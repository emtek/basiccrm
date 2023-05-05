CREATE MIGRATION m13jzejnzzto3s4xeauxpduyydtggadfbsfpyutv6quj3dnmztldwq
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE ABSTRACT TYPE default::Auditable {
      CREATE REQUIRED PROPERTY created -> std::datetime;
  };
  CREATE SCALAR TYPE default::OpportunityStatus EXTENDING enum<New, ClosedWon, ClosedLost>;
  CREATE TYPE default::Opportunity EXTENDING default::Auditable {
      CREATE REQUIRED PROPERTY name -> std::str;
      CREATE REQUIRED PROPERTY status -> default::OpportunityStatus;
  };
  CREATE SCALAR TYPE default::CustomerStatus EXTENDING enum<Active, NonActive, Lead>;
  CREATE TYPE default::Customer EXTENDING default::Auditable {
      CREATE MULTI LINK opportunities -> default::Opportunity;
      CREATE REQUIRED PROPERTY email -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY name -> std::str;
      CREATE REQUIRED PROPERTY status -> default::CustomerStatus;
  };
};
