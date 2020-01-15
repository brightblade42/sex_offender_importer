Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select ID
     ,cast(name as TEXT) as name
     ,cast(age as TEXT) as dateOfBirth
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(cast(state as TEXT)) as state

     -- aliases
     ,json_array("Unknown") as aliases
     --addresses
      ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          || ' ' || cast(state AS TEXT),
                                  'type', cast(AddressType as TEXT)
                          ))

       FROM (SELECT Address,state, AddressType
             FROM ALSexOffenders_addresses arad
             where arad.ID = ALSexOffenders_main.ID
               and arad.state = ALSexOffenders_main.state)
)as addresses

     --offenses
    , (SELECT json_group_array(json_object ( 'offense', cast(offense as Text) ))
       FROM (SELECT description as offense
             FROM ALSexOffenders_offenses offz
             WHERE offz.id = ALSexOffenders_main.id
               and ALSexOffenders_main.state = offz.state
            )
) as offenses
  ,(select json_group_array( cast(smt as text))
       from (select scars_marks_tattoos as smt from ALSexOffenders_smts smts
             where smts.id = ALSexOffenders_main.id
               and smts.state = ALSexOffenders_main.state)) as scarsTattoos

  ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from ALSexOffenders_photos stphotos
             where stphotos.id = ALSexOffenders_main.id
               and stphotos.state = ALSexOffenders_main.state)) as photos

from ALSexOffenders_main;
