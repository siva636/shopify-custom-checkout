use super::schema;
use std::borrow::Cow;

use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_transform_run(input: schema::run::Input) -> Result<schema::CartTransformRunResult> {
  let cart_operations: Vec<schema::Operation> = get_expand_cart_operations(&input.cart());

  Ok(schema::CartTransformRunResult {
    operations: cart_operations,
  })
}

// expand operation logic

fn get_expand_cart_operations(cart: &schema::run::input::Cart) -> Vec<schema::Operation> {
  let mut result: Vec<schema::Operation> = Vec::new();

  for line in cart.lines().iter() {
    let variant = match &line.merchandise() {
      schema::run::input::cart::lines::Merchandise::ProductVariant(variant) => Some(variant),
      _ => None,
    };
    if variant.is_none() {
      continue;
    }

    if let Some(merchandise) = &variant {
      let component_references = get_component_references(&merchandise);

      if component_references.is_empty() {
        continue;
      }

      let mut expand_relationships: Vec<schema::ExpandedItem> = Vec::new();

      for reference in component_references.iter() {
        let expand_relationship = schema::ExpandedItem {
          merchandise_id: reference.clone(),
          quantity: 1,
          price: None,
          attributes: None,
        };

        expand_relationships.push(expand_relationship);
      }

      // NOTE: type is LineExpandOperation, variant is Operation::LineExpand
      let expand_operation = schema::LineExpandOperation {
        cart_line_id: line.id().clone(),
        expanded_cart_items: expand_relationships,
        price: None,
        image: None,
        title: None,
      };

      result.push(schema::Operation::LineExpand(expand_operation));
    }
  }

  result
}

pub type ComponentReferences = Vec<schema::Id>;

fn get_component_references(
  variant: &schema::run::input::cart::lines::merchandise::ProductVariant,
) -> Cow<[schema::Id]> {
  if let Some(component_reference_metafield) = &variant.component_reference() {
    return component_reference_metafield.json_value().into();
  }

  Vec::new().into()
}