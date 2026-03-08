use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<schema::CartLinesDiscountsGenerateRunResult> {
    const DISCOUNT_PERCENTAGE: f64 = 25.0;
    const DISCOUNT_MESSAGE: &str = "💎25% OFF";

    let cart = input.cart();
    let mut has_premium = false;
    let mut has_featured = false;
    let mut ops = vec![];

    for line in cart.lines().iter() {
        if let schema::cart_lines_discounts_generate_run::input::cart::lines::Merchandise::ProductVariant(
            variant,
        ) = &line.merchandise()
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

    if !has_premium || !has_featured {
        return Ok(schema::CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    for line in cart.lines().iter() {
        let variant = match &line.merchandise() {
            schema::cart_lines_discounts_generate_run::input::cart::lines::Merchandise::ProductVariant(v) => v,
            _ => continue,
        };

        let product = variant.product();

        let is_premium = *product.in_premium_collection();
        let is_featured = *product.in_featured_collection();

        if !is_premium && !is_featured {
            continue;
        }

        let op = schema::ProductDiscountCandidateTarget::CartLine(schema::CartLineTarget {
            id: line.id().clone(),
            quantity: None,
        });

        ops.push(op);
    }

    let has_order_discount_class = input
        .discount()
        .discount_classes()
        .contains(&schema::DiscountClass::Order);
    let has_product_discount_class = input
        .discount()
        .discount_classes()
        .contains(&schema::DiscountClass::Product);

    if !has_order_discount_class && !has_product_discount_class {
        return Ok(schema::CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut operations = vec![];

    // Check if the discount has the PRODUCT class
    if has_product_discount_class {
        operations.push(schema::CartOperation::ProductDiscountsAdd(
            schema::ProductDiscountsAddOperation {
                selection_strategy: schema::ProductDiscountSelectionStrategy::First,
                candidates: vec![schema::ProductDiscountCandidate {
                    targets: ops,
                    message: Some(DISCOUNT_MESSAGE.to_string()),
                    value: schema::ProductDiscountCandidateValue::Percentage(schema::Percentage {
                        value: Decimal(DISCOUNT_PERCENTAGE),
                    }),
                    associated_discount_code: None,
                }],
            },
        ));
    }

    Ok(schema::CartLinesDiscountsGenerateRunResult { operations })
}
