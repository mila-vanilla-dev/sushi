/*
Public, customer-facing:

| Method | Endpoint                         | Purpose                                                                                                                          |
| ------ | -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `POST` | `/api/orders`                    | Create a new print order — includes customer info, selected print sizes, quantities, uploaded image references, shipping option. |
| `POST` | `/api/orders/:order_id/upload`   | Upload one or more image files for a specific order (return URLs or IDs for stored images).                                      |
| `GET`  | `/api/orders/:order_id`          | Retrieve order status and details (customer view).                                                                               |
| `GET`  | `/api/prints/sizes`              | Get available print sizes, prices, and descriptions.                                                                             |
| `POST` | `/api/shipping/quote`            | Get a live shipping cost based on address, package weight, and size.                                                             |
| `POST` | `/api/payments/intent`           | Create a payment intent (PayPal) for an order.                                                                            |
| `POST` | `/api/payments/webhook`          | Handle payment provider webhooks (order paid, failed, refunded).                                                                 |
| `GET`  | `/api/orders/:order_id/tracking` | Get shipping tracking info (pulled from shipping API).                                                                           |
*/
// TODO: Implement orders api
pub mod orders;
// TODO: Implement prints api
pub mod prints;
// TODO: Implement shipping api
pub mod shipping;
// TODO: Implement payments api
pub mod payments;
// TODO: Implement auth api
pub mod auth;

/*
Admin only, private:

| Method   | Endpoint                                     | Purpose                                                                        |
| -------- | -------------------------------------------- | ------------------------------------------------------------------------------ |
| `GET`    | `/api/admin/orders`                          | List all orders with filters for status (pending, in progress, shipped, etc.). |
| `GET`    | `/api/admin/orders/:order_id`                | Get full details, uploaded files, shipping info for a single order.            |
| `PATCH`  | `/api/admin/orders/:order_id/status`         | Update an order status (pending → in progress → shipped).                      |
| `POST`   | `/api/admin/orders/:order_id/shipping-label` | Generate a shipping label via EasyPost/Shippo API.                             |
| `PATCH`  | `/api/admin/orders/:order_id/tracking`       | Update tracking info if manual.                                                |
| `POST`   | `/api/admin/prints/sizes`                    | Add a new print size & price.                                                  |
| `PATCH`  | `/api/admin/prints/sizes/:size_id`           | Edit an existing print size/price.                                             |
| `DELETE` | `/api/admin/prints/sizes/:size_id`           | Remove a print size from the catalog.                                          |
*/
// TODO: Implement admin api
pub mod admin;
