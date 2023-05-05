CREATE MIGRATION m1cekgqxtdt7c3wixwfbku4eodg5qybmslliofssh3vrixy32ykska
    ONTO m1lzjb7qefusvo6ur55wj7avdzppbh4q7uhqz5lbfd6a5ew6ser4ia
{
  ALTER TYPE default::Opportunity {
      ALTER LINK customer {
          SET REQUIRED USING (SELECT
              default::Customer 
          LIMIT
              1
          );
      };
  };
};
