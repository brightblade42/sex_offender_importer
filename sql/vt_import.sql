Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,Name
     ,DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     --aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM VTSexOffenders_aliases als
            WHERE als.id = VTSexOffenders_main.id
              AND VTSexOffenders_main.state = als.state
           )
) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as Text))) as addresses

     --offenses

     ,(SELECT
           json_group_array ( json_object ('offense', cast(offense as TEXT) ))
       FROM

           (SELECT
                OffenseDescription as offense
            FROM VTSexOffenders_offenses aro where VTSexOffenders_main.ID = aro.ID
                                               and VTSexOffenders_main.state = aro.state
           )) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from VTSexOffenders_smts smts
             where smts.id = VTSexOffenders_main.id
               and smts.state = VTSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from VTSexOffenders_photos azp
             where azp.id = VTSexOffenders_main.id
               and azp.state = VTSexOffenders_main.state)) as photos


From VTSexOffenders_main
