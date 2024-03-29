Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
------------- FL
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM FL_SexOffenders_aliases als
             WHERE als.id = FL_SexOffenders_main.id
               AND FL_SexOffenders_main.state = als.state)
) as "aliases"

     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(street as TEXT)
                       || ' ' || cast(CityStateZip as TEXT)
                       ))
       FROM
           (SELECT street,
                   CityStateZip
            FROM FL_SexOffenders_addresses arad
            where arad.ID = FL_SexOffenders_main.ID
              and arad.state = FL_SexOffenders_main.state

           )) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', offense)
               )
       FROM

           (SELECT
                cast(CrimeDescription as TEXT) as offense
            FROM FL_SexOffenders_offenses aro
            where FL_SexOffenders_main.ID = aro.ID
              and FL_SexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(type as text) || ' ' || cast(number as Text) || ' ' || cast(location as Text))
       from (select type, location, number  from FL_SexOffenders_smts smts
             where smts.id = FL_SexOffenders_main.id
               and smts.state = FL_SexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from FL_SexOffenders_photos azp
             where azp.id = FL_SexOffenders_main.id
               and azp.state = FL_SexOffenders_main.state)) as photos


from FL_SexOffenders_main
