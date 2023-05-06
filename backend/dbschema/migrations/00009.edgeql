CREATE MIGRATION m13hsrd2otf2olx37fydj4uv2rrsorxsfkocmnzlg2curxadw5xq3q
    ONTO m1a4qugordlt5p2kqbe5abpijicgekqefqk74t5zajyrf4m6qfrfra
{
  ALTER TYPE default::Customer {
      ALTER LINK opportunities {
          ON SOURCE DELETE DELETE TARGET;
      };
  };
};
