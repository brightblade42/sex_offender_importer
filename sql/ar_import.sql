Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
-- AR --------------------------------------------------------------------
SELECT id
     ,name
     ,DOB  as DateOfBirth
     ,eyes
     ,hair
     ,'' as height
     ,'' as weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM AR_sex_offender_alias als
            WHERE als.id = AR_sex_offender_main.id
              AND AR_sex_offender_main.state = als.state
           )
) as aliases
     -- addresses
     ,json_array(
        json_object('address', cast(address as Text))) as addresses

     -- offenses
     ,json_array(
        json_object('offense', cast(Offense as Text))) as offenses

     ,json_array(cast(ScarsTattoos as Text)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(Photo as Text))
       from (select Photo from AR_sex_offender_photos azp
             where azp.id = AR_sex_offender_main.id
               and azp.state = AR_sex_offender_main.state)) as photos


from AR_sex_offender_main
