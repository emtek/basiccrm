CREATE MIGRATION m1vc5egmlykbeqzfordqd7eeex4azxq6oejeuqck225vv75otkoo7a
    ONTO m13jzejnzzto3s4xeauxpduyydtggadfbsfpyutv6quj3dnmztldwq
{
  ALTER TYPE default::Customer {
      ALTER PROPERTY status {
          SET default := (default::CustomerStatus.Active);
      };
  };
  ALTER TYPE default::Opportunity {
      CREATE REQUIRED LINK customer -> default::Customer {
          SET REQUIRED USING (SELECT
              default::Customer 
          LIMIT
              1
          );
      };
      ALTER PROPERTY status {
          SET default := (default::OpportunityStatus.New);
      };
  };
};
