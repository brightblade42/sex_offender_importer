Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
-------------DE
select id
     ,name
     ,BirthDate as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM DE_SexOffenders_aliases als
             WHERE als.id = DE_SexOffenders_main.id
               AND DE_SexOffenders_main.state = als.state)
) as aliases

     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Street as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ,'type',cast(Type as TEXT)))
       FROM
           (SELECT street,
                   CityStateZip,
                   Type
            FROM DE_SexOffenders_addresses arad
            where arad.ID = DE_SexOffenders_main.ID
              and arad.state = DE_SexOffenders_main.state

           )) as addresses

     --offenses

    ,(SELECT
          json_group_array (
                  json_object ('offense', cast(offense as TEXT))
              )
      FROM

          (SELECT
               Description as offense
           FROM DE_SexOffenders_convictions aro
           where DE_SexOffenders_main.ID = aro.ID
             and DE_SexOffenders_main.state = aro.state
          )
) as offenses


     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from DE_SexOffenders_smts smts
             where smts.id = DE_SexOffenders_main.id
               and smts.state = DE_SexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from DE_SexOffenders_photos azp
             where azp.id = DE_SexOffenders_main.id
               and azp.state = DE_SexOffenders_main.state)) as photos


from DE_SexOffenders_main
