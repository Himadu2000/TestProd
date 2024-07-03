-- Namespace for CHost
DEFINE NAMESPACE CHost;
-- Specify the namespace for the database
USE NS CHost;
-- Define database
DEFINE DATABASE Cezerin;
-- Create a schemafull store table.
DEFINE TABLE store SCHEMAFULL;
-- Define some fields.
DEFINE FIELD name ON TABLE store TYPE string;
DEFINE FIELD users ON TABLE store TYPE set<object>;
DEFINE FIELD users.name ON TABLE store TYPE string;
DEFINE FIELD users.email ON TABLE store TYPE string ASSERT string::is::email($value);
DEFINE FIELD users.scopes ON TABLE store TYPE set<string>;
-- Create a schemafull product table.
DEFINE TABLE product SCHEMAFULL;
-- Define some fields.
DEFINE FIELD name ON TABLE product TYPE string;
DEFINE FIELD description ON TABLE product TYPE string;
DEFINE FIELD slug ON TABLE product TYPE string VALUE string::slug($value);
DEFINE FIELD meta_title ON TABLE product TYPE string;
DEFINE FIELD meta_description ON TABLE product TYPE string;
DEFINE FIELD regular_price ON TABLE product TYPE float;
DEFINE FIELD sale_price ON TABLE product TYPE float;
DEFINE FIELD date_sale_from ON TABLE product TYPE datetime;
DEFINE FIELD date_sale_to ON TABLE product TYPE datetime;
DEFINE FIELD sku ON TABLE product TYPE string;
DEFINE FIELD stock_quantity ON TABLE product TYPE int;
DEFINE FIELD weight ON TABLE product TYPE float;
DEFINE FIELD date_stock_expected ON TABLE product TYPE datetime;
DEFINE FIELD stock_tracking ON TABLE product TYPE bool;
DEFINE FIELD stock_preorder ON TABLE product TYPE bool;
DEFINE FIELD stock_backorder ON TABLE product TYPE bool;
DEFINE FIELD discontinued ON TABLE product TYPE bool;
DEFINE FIELD enabled ON TABLE product TYPE bool;
DEFINE FIELD attributes ON TABLE product TYPE set<object>;
DEFINE FIELD attributes.name ON TABLE product TYPE string;
DEFINE FIELD attributes.value ON TABLE product TYPE string;
DEFINE FIELD variants ON TABLE product TYPE set<object>;
DEFINE FIELD variants.name ON TABLE product TYPE string;
DEFINE FIELD variants.control ON TABLE product TYPE set<string>;
DEFINE FIELD variants.required ON TABLE product TYPE bool;
DEFINE FIELD variants.values ON TABLE product TYPE set<string>;
DEFINE FIELD category_ids ON TABLE product TYPE set<record<string>>;
DEFINE FIELD tags ON TABLE product TYPE set<string>;
DEFINE FIELD position ON TABLE product TYPE int;
DEFINE FIELD related_products ON TABLE product TYPE set<record<string>>;
DEFINE FIELD images ON TABLE product TYPE set<object>;
