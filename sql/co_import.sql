Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(Gender as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM CO_SexOffenders_aliases als
             WHERE als.id = CO_SexOffenders_main.id
               AND CO_SexOffenders_main.state = als.state)
) as "aliases"

     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(address as TEXT)
                       || ' ' || cast(AddressExt as TEXT)
                       || ' ' || cast(CityZip as TEXT)
                       ))
       FROM
           (SELECT address,
                   AddressExt,
                   CityZip
            FROM CO_SexOffenders_addresses arad
            where arad.ID = CO_SexOffenders_main.ID
              and arad.state = CO_SexOffenders_main.state

           )) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT))
               )
       FROM

           (SELECT
                Description as offense
            FROM CO_SexOffenders_convictions aro
            where CO_SexOffenders_main.ID = aro.ID
              and CO_SexOffenders_main.state = aro.state
           )
) as offenses
     -- scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarMarkTattoo as smt from CO_SexOffenders_smts smts
             where smts.id = CO_SexOffenders_main.id
               and smts.state = CO_SexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from CO_SexOffenders_photos azp
             where azp.id = CO_SexOffenders_main.id
               and azp.state = CO_SexOffenders_main.state)) as photos


from CO_SexOffenders_main
