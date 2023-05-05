CREATE MIGRATION m15n6dwvhx5ou676lrtq6dl36xzuwc5zgyvjzonco4qgut4lu5fxgq
    ONTO m1pj7lmpithav6l7ayde23v4o2uwga4cnb7nu6pmk4a3we3zxplvyq
{
  ALTER TYPE default::Customer {
      ALTER LINK opportunities {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::Opportunity {
      DROP LINK customer;
  };
};
