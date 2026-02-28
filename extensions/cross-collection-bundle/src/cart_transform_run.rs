use crate::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

const DISCOUNT_PERCENTAGE: f64 = 25.0;

#[shopify_function]
fn cart_transform_run(
    input: schema::cart_transform_run::Input,
) -> Result<schema::CartTransformRunResult> {

    let cart = input.cart();

    // First pass: determine if we have at least one premium and one featured item
    let mut has_premium = false;
    let mut has_featured = false;

    for line in cart.lines().iter() {
        if let schema::cart_transform_run::input::cart::lines::Merchandise::ProductVariant(variant) =
            &line.merchandise()
        {
            let product = variant.product();
            if *product.in_premium_collection() {
                has_premium = true;
            }
            if *product.in_featured_collection() {
                has_featured = true;
            }
        }
    }

    // If we don't have at least one of each, don't apply any discount
    if !has_premium || !has_featured {
        return Ok(schema::CartTransformRunResult {
            operations: vec![],
        });
    }

    // Both sets are present: build lineUpdate operations
    let discount_factor = 1.0 - (DISCOUNT_PERCENTAGE / 100.0);

    let mut operations: Vec<schema::Operation> = Vec::new();

    for line in cart.lines().iter() {
        let variant = match &line.merchandise() {
            schema::cart_transform_run::input::cart::lines::Merchandise::ProductVariant(v) => v,
            _ => continue, // Ignore CustomProduct, etc.
        };

        let product = variant.product();

        let is_premium = *product.in_premium_collection();
        let is_featured = *product.in_featured_collection();

        if !is_premium && !is_featured {
            continue;
        }

        // Current unit price in presentment currency
        let current_amount = line
            .cost()
            .amount_per_quantity()
            .amount()
            .0; // Decimal(f64) -> get f64

        // Apply percentage discount
        let discounted_amount = current_amount * discount_factor;

        // Defensive: ensure non-negative adjustment
        if discounted_amount < 0.0 {
            continue;
        }

        let price_adjustment = schema::LineUpdateOperationPriceAdjustment {
            adjustment:
                schema::LineUpdateOperationPriceAdjustmentValue::FixedPricePerUnit(
                    schema::LineUpdateOperationFixedPricePerUnitAdjustment {
                        amount: Decimal::from(discounted_amount),
                    },
                ),
        };

        let op = schema::LineUpdateOperation {
            cart_line_id: line.id().clone(),
            image: None,
            title: None,
            price: Some(price_adjustment),
        };

        operations.push(schema::Operation::LineUpdate(op));
    }

    Ok(schema::CartTransformRunResult { operations })
}

