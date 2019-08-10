Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(birth_date as TEXT)   as DateOfBirth
     ,cast(eye_color as TEXT)   as eyes
     ,cast(hair_color as TEXT)   as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(gender as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     -- aliases
     , (SELECT json_group_array(cast(alias as TEXT))

        FROM (SELECT alias_name as alias
              FROM ky_Sex_Offenders_aliases als
              WHERE als.id = ky_Sex_Offenders_main.id
                AND ky_Sex_Offenders_main.state = als.state
             )

) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT))
                  )

       FROM (SELECT Address
             FROM ky_Sex_Offenders_addresses arad
             where arad.ID = ky_Sex_Offenders_main.ID
               and arad.state = ky_Sex_Offenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT victim_info as offense
            FROM ky_sex_offenders_victim_info aro
            where ky_Sex_Offenders_main.ID = aro.ID
              and ky_Sex_Offenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,json_array("NoneReported") as scarsTattoos
    /*,(select json_group_array( cast(smt as text))

      from (select ScarsMarksTattoos as smt from CTSexOffenders_smts smts
            where smts.id = CTSexOffenders_main.id
              and smts.state = CTSexOffenders_main.state)) as scarsTattoos

     */
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from ky_Sex_Offenders_photos azp
             where azp.id = ky_Sex_Offenders_main.id
               and azp.state = ky_Sex_Offenders_main.state)) as photos

from ky_Sex_Offenders_main
