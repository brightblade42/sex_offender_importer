Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,ifnull(cast(Name as TEXT),'') as Name
     ,cast(DOB as TEXT) as DateOfBirth

     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,'' as race --no race listed
     ,cast(sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     --alias
     ,json_array("None Reported") as aliases
     --address
     ,json_array(
        json_object('address', cast(Residence_Address as TEXT))) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT))
               )
       FROM

           (SELECT
                offenseName as offense
            FROM ORSexOffenders_offenses aro where ORSexOffenders_main.ID = aro.ID
                                               and ORSexOffenders_main.state = aro.state
           )
) as offenses

     ,(select json_group_array( cast(smt as text))
       from (select smt from ORSexOffenders_smts smts
             where smts.id = ORSexOffenders_main.id
               and smts.state = ORSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from ORSexOffenders_photos azp
             where azp.id = ORSexOffenders_main.id
               and azp.state = ORSexOffenders_main.state)) as photos


From ORSexOffenders_main
