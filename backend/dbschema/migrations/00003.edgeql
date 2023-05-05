CREATE MIGRATION m1pj7lmpithav6l7ayde23v4o2uwga4cnb7nu6pmk4a3we3zxplvyq
    ONTO m1vc5egmlykbeqzfordqd7eeex4azxq6oejeuqck225vv75otkoo7a
{
  ALTER TYPE default::Auditable {
      ALTER PROPERTY created {
          SET default := (std::datetime_current());
      };
  };
};
