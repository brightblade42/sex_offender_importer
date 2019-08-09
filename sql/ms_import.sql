
Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name
     ,DateOfBirth
     ,Eyes
     ,Hair
     ,Height
     ,Weight
     ,race
     ,gender as sex
     ,trim(state) as state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MS_SexOffenders_aliases als
             WHERE als.id = MS_SexOffenders_main.id
               AND MS_SexOffenders_main.state = als.state)
) as aliases
     ,json_array(json_object('address', cast(PrimaryAddress as TEXT))) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT  offense
            FROM MS_SexOffenders_offenses aro
            where MS_SexOffenders_main.ID = aro.ID
              and MS_SexOffenders_main.state = aro.state
           )
) as offenses
     --scars tattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarTattoo as smt from MS_SexOffenders_smts smts
             where smts.id = MS_SexOffenders_main.id
               and smts.state = MS_SexOffenders_main.state)) as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MS_SexOffenders_photos azp
             where azp.id = MS_SexOffenders_main.id
               and azp.state = MS_SexOffenders_main.state)) as photos

from MS_SexOffenders_main
