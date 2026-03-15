import "@shopify/ui-extensions/preact";
import { render } from "preact";
import { useEffect, useState } from "preact/hooks";

export default function extension() {
  render(<Extension />, document.body);
}

function Extension() {
  const { query, lines } = shopify;
  const [data, setData] = useState();
  let hasPremium = false;
  let hasFeatured = false;

  useEffect(() => {
    const productIds = lines.value.map((line) => line.merchandise.product.id);

    shopify
      .query(
        `query ProductTags($ids: [ID!]!) {
          nodes(ids: $ids) {
            ... on Product {
              id
              tags
            }
          }
        }
      `,
        { variables: { ids: productIds } },
      )
      .then(({ data, errors }) => {
        // console.log("data", data?.nodes);
        return setData(data);
      })
      .catch(console.error);
  }, [query, lines]);

  data?.nodes.forEach((node) => {
    if (node?.tags.includes("Premium") || node?.tags.includes("premium")) {
      hasPremium = true;
    }
    if (node?.tags.includes("Featured") || node?.tags.includes("featured")) {
      hasFeatured = true;
    }
  });

  return (
    <>
      {hasPremium && hasFeatured && (
        <s-banner heading="Pick & Match offer applied" tone="success" />
      )}
      {!hasPremium && !hasFeatured && (
        <s-banner
          heading="Add at least one each from Premium and Featured collections to get Pick & Match offer"
          tone="warning"
        />
      )}
      {hasPremium && !hasFeatured && (
        <s-banner
          heading="Add at least one Featured product to get Pick & Match offer"
          tone="warning"
        />
      )}
      {!hasPremium && hasFeatured && (
        <s-banner
          heading="Add at least one Premium product to get Pick & Match offer"
          tone="warning"
        />
      )}
    </>
  );
}
