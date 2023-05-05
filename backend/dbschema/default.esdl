module default {
 abstract type Auditable{
    required property created -> datetime{
        default := datetime_current();
    }
 }

 scalar type CustomerStatus extending enum<Active, NonActive, Lead>;
 
 type Customer extending Auditable {
    required property name -> str;
    required property email -> str {
        constraint exclusive;
    };
    required property status -> CustomerStatus{
        default := CustomerStatus.Active;
    }
    multi link opportunities -> Opportunity {
        constraint exclusive;
        on target delete allow;
    }
 }

 scalar type OpportunityStatus extending enum<New, ClosedWon, ClosedLost>;

 type Opportunity extending Auditable {
    link customer := .<opportunities[is Customer];
    required property name -> str;
    required property status -> OpportunityStatus{
        default := OpportunityStatus.New;
    }
 }
}
