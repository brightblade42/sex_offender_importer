Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(age as TEXT)    as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM AZ_SexOffenders_aliases als
            WHERE als.id = AZ_SexOffenders_main.id
              AND AZ_SexOffenders_main.state = als.state
           )
) as aliases
     -- addresses
     ,json_array(
        json_object('address', cast(address as Text))) as addresses

     -- offenses
     ,(SELECT json_group_array(json_object ( 'offense', cast(offense as Text) ))
       FROM (SELECT description as offense
             FROM AZ_SexOffenders_offenses azo
             WHERE azo.id = AZ_SexOffenders_main.id
               and AZ_SexOffenders_main.state = azo.state
            )
) as offenses

     ,json_array(cast(scars_tattoos as Text)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from AZ_SexOffenders_photos azp
             where azp.id = AZ_SexOffenders_main.id
               and azp.state = AZ_SexOffenders_main.state)) as photos


from AZ_SexOffenders_main
