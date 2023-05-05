CREATE MIGRATION m1lzjb7qefusvo6ur55wj7avdzppbh4q7uhqz5lbfd6a5ew6ser4ia
    ONTO m15n6dwvhx5ou676lrtq6dl36xzuwc5zgyvjzonco4qgut4lu5fxgq
{
  ALTER TYPE default::Opportunity {
      CREATE LINK customer -> default::Customer;
  };
};
