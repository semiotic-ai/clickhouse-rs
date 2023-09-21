use std::sync::Arc;

use crate::{
    binary::{Encoder, ReadEx},
    errors::Result,
    types::{
        column::{
            column_data::{ArcColumnData, BoxColumnData},
            ArcColumnWrapper, ColumnData,
        },
        SqlType, Value, ValueRef,
    },
};

use chrono_tz::Tz;

pub(crate) struct LowCardinalityColumnData {
    pub(crate) inner: ArcColumnData,
}

impl LowCardinalityColumnData {
    pub(crate) fn load<R: ReadEx>(
        reader: &mut R,
        type_name: &str,
        size: usize,
        tz: Tz,
    ) -> Result<Self> {
        let inner =
            <dyn ColumnData>::load_data::<ArcColumnWrapper, _>(reader, type_name, size, tz)?;
        Ok(LowCardinalityColumnData { inner })
    }
}

impl ColumnData for LowCardinalityColumnData {
    fn sql_type(&self) -> SqlType {
        let inner_type = self.inner.sql_type();
        SqlType::LowCardinality(inner_type.into())
    }

    fn save(&self, encoder: &mut Encoder, start: usize, end: usize) {
        self.inner.save(encoder, start, end);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn push(&mut self, value: Value) {
        let inner_column: &mut dyn ColumnData = Arc::get_mut(&mut self.inner).unwrap();
        inner_column.push(value);
    }

    fn at(&self, index: usize) -> ValueRef {
        self.inner.at(index)
    }

    fn clone_instance(&self) -> BoxColumnData {
        Box::new(Self {
            inner: self.inner.clone(),
        })
    }

    unsafe fn get_internal(&self, pointers: &[*mut *const u8], level: u8, props: u32) -> Result<()> {
        self.inner.get_internal(pointers, level, props)
    }

    fn cast_to(&self, _this: &ArcColumnData, target: &SqlType) -> Option<ArcColumnData> {
        if let SqlType::LowCardinality(inner_target) = target {
            if let Some(inner) = self.inner.cast_to(&self.inner, inner_target) {
                return Some(Arc::new(LowCardinalityColumnData {
                    inner,
                }));
            }
        }
        None
    }
}
