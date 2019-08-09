Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,name
     ,DOB  as DateOfBirth
     ,'' as eyes
     ,'' as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM ILSexOffenders_aliases als
            WHERE als.id = ILSexOffenders_main.id
              AND ILSexOffenders_main.state = als.state
           )
) as aliases
     -- addresses
     ,json_array(
        json_object('address', cast(address as Text) || ' ' || cast(City as TEXT))) as addresses

     -- offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT Crime as offense
            FROM ILSexOffenders_crimes aro
            where ILSexOffenders_main.ID = aro.ID
              and ILSexOffenders_main.state = aro.state
           )
) as offenses
     ,json_array("Unknown") as scarsTattoos
     --photos
     ,(select json_group_array(cast(Photo as Text))
       from (select Photo from AR_sex_offender_photos azp
             where azp.id = ILSexOffenders_main.id
               and azp.state = ILSexOffenders_main.state)) as photos


from ILSexOffenders_main
