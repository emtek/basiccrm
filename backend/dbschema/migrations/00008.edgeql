CREATE MIGRATION m1a4qugordlt5p2kqbe5abpijicgekqefqk74t5zajyrf4m6qfrfra
    ONTO m12gw5b7sl6q6t5o2zjhaioutrguqnxfv3myt4udw6nhxw5gbn5sqq
{
  ALTER TYPE default::Opportunity {
      ALTER LINK customer {
          USING (.<opportunities[IS default::Customer]);
          RESET OPTIONALITY;
      };
  };
};
