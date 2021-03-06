use arrow::array::{ArrayBuilder, BooleanBuilder, Float64Builder, StringBuilder, UInt64Builder};
use arrow::datatypes::DataType as ArrowDataType;
use arrow::datatypes::Field;

/// Associate arrow builder with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send;

    fn builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self);
    fn field(header: &str) -> Field;
}

impl ArrowAssoc for u64 {
    type Builder = UInt64Builder;

    fn builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    fn append(builder: &mut UInt64Builder, value: u64) {
        builder.append_value(value).unwrap();
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, false)
    }
}

impl ArrowAssoc for Option<u64> {
    type Builder = UInt64Builder;

    fn builder(nrows: usize) -> UInt64Builder {
        UInt64Builder::new(nrows)
    }

    fn append(builder: &mut UInt64Builder, value: Option<u64>) {
        builder.append_option(value).unwrap();
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, true)
    }
}

impl ArrowAssoc for f64 {
    type Builder = Float64Builder;

    fn builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: f64) {
        builder.append_value(value).unwrap();
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, false)
    }
}

impl ArrowAssoc for bool {
    type Builder = BooleanBuilder;

    fn builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: bool) {
        builder.append_value(value).unwrap();
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, false)
    }
}

impl ArrowAssoc for String {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> StringBuilder {
        StringBuilder::new(nrows)
    }

    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str()).unwrap();
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}
