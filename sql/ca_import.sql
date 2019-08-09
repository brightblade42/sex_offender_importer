Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,name
     ,'' as DateOfBirth
     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,Ethnicity as race
     ,sex
     ,trim(state) as state

     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM CA_SexOffenders_alias als
            WHERE als.id = CA_SexOffenders_main.id
              AND CA_SexOffenders_main.state = als.state
           )
) as aliases
     --addresses
     ,json_array(
        json_object('address', cast(address as Text))) as addresses

     -- offenses
     ,(SELECT
           json_group_array(json_object ( 'offense', cast(offense as Text)
               ))
       FROM (SELECT OffenseDescription as offense
             FROM CA_SexOffenders_offenses azo
             WHERE azo.id = CA_SexOffenders_main.id
               and CA_SexOffenders_main.state = azo.state
            )
) as offenses
     --scars tattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from CA_SexOffenders_smts smts
             where smts.id = CA_SexOffenders_main.id
               and smts.state = CA_SexOffenders_main.state)) as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from CA_SexOffenders_photos azp
             where azp.id = CA_SexOffenders_main.id
               and azp.state = CA_SexOffenders_main.state)) as photos


from CA_SexOffenders_main
