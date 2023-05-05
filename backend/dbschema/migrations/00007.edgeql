CREATE MIGRATION m12gw5b7sl6q6t5o2zjhaioutrguqnxfv3myt4udw6nhxw5gbn5sqq
    ONTO m1cekgqxtdt7c3wixwfbku4eodg5qybmslliofssh3vrixy32ykska
{
  ALTER TYPE default::Customer {
      ALTER LINK opportunities {
          ON TARGET DELETE ALLOW;
      };
  };
};
