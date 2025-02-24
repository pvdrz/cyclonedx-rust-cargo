/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{
    external_models::normalized_string::NormalizedString,
    validation::{Validate, ValidationContext, ValidationPathComponent, ValidationResult},
};

/// Represents a name-value store that can be used to describe additional data about the components, services, or the BOM that
/// isn’t native to the core specification.
///
/// Defined via the [XML schema](https://cyclonedx.org/docs/1.3/xml/#type_propertyType). Please see the
/// [CycloneDX use case](https://cyclonedx.org/use-cases/#properties--name-value-store) for more information and examples.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Properties(pub Vec<Property>);

impl Validate for Properties {
    fn validate_with_context(&self, context: ValidationContext) -> ValidationResult {
        let mut results: Vec<ValidationResult> = vec![];

        for (index, property) in self.0.iter().enumerate() {
            let property_context =
                context.extend_context(vec![ValidationPathComponent::Array { index }]);
            results.push(property.validate_with_context(property_context));
        }

        results
            .into_iter()
            .fold(ValidationResult::default(), |acc, result| acc.merge(result))
    }
}

/// Represents an individual property with a name and value
///
/// Defined via the [XML schema](https://cyclonedx.org/docs/1.3/xml/#type_propertyType)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub value: NormalizedString,
}

impl Property {
    /// Constructs a `Property` with a name and value
    /// ```
    /// use cyclonedx_bom::models::property::Property;
    ///
    /// let property = Property::new("Foo", "Bar");
    /// ```
    pub fn new(name: impl ToString, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: NormalizedString::new(value),
        }
    }
}

impl Validate for Property {
    fn validate_with_context(&self, context: ValidationContext) -> ValidationResult {
        let mut results: Vec<ValidationResult> = vec![];

        let value_context = context.with_struct("Property", "value");

        results.push(self.value.validate_with_context(value_context));

        results
            .into_iter()
            .fold(ValidationResult::default(), |acc, result| acc.merge(result))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::validation::FailureReason;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_pass_validation() {
        let validation_result = Properties(vec![Property {
            name: "property name".to_string(),
            value: NormalizedString("property value".to_string()),
        }])
        .validate();

        assert_eq!(validation_result, ValidationResult::Passed);
    }

    #[test]
    fn it_should_fail_validation() {
        let validation_result = Properties(vec![Property {
            name: "property name".to_string(),
            value: NormalizedString("spaces and \ttabs".to_string()),
        }])
        .validate();

        assert_eq!(
            validation_result,
            ValidationResult::Failed {
                reasons: vec![FailureReason {
                    message: "NormalizedString contains invalid characters \\r \\n \\t or \\r\\n"
                        .to_string(),
                    context: ValidationContext(vec![
                        ValidationPathComponent::Array { index: 0 },
                        ValidationPathComponent::Struct {
                            struct_name: "Property".to_string(),
                            field_name: "value".to_string(),
                        },
                    ]),
                }],
            }
        );
    }
}
