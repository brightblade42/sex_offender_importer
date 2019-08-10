Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,cast(Name as TEXT) as Name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
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
