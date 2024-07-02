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
DEFINE FIELD users ON TABLE store TYPE array < object >;
DEFINE FIELD users.name ON TABLE store TYPE string;
DEFINE FIELD users.email ON TABLE store TYPE string ASSERT string::is::email($value);
DEFINE FIELD users.scopes ON TABLE store TYPE array;
-- Create a schemafull product table.
DEFINE TABLE product SCHEMAFULL;
-- Define some fields.
DEFINE FIELD firstName ON TABLE product TYPE string;
DEFINE FIELD lastName ON TABLE product TYPE string;
DEFINE FIELD email ON TABLE product TYPE string;
ASSERT string::is::email($value);