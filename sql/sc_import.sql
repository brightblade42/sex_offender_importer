Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,FirstName || ' ' || MiddleName || ' ' || LastName as name
     ,DateOfBirth

     ,eyes
     ,hair
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
             FROM SCSexOffenders_aliases als
             WHERE als.id = SCSexOffenders_main.id
               AND SCSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(AddressLine1 as TEXT) || ' ' || cast(AddressLine2 as TEXT)
                          ))

       FROM (SELECT AddressLine1, AddressLine2
             FROM SCSexOffenders_addresses arad
             where arad.ID = SCSexOffenders_main.ID
               and arad.state = SCSexOffenders_main.state)
)as addresses
     --offenses

     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(Offense as Text)))
       FROM

           (SELECT Offense
            FROM SCSexOffenders_offenses aro
            where SCSexOffenders_main.ID = aro.ID
              and SCSexOffenders_main.state = aro.state
           )
) as offenses


     --scarsTattoos
     ,(select json_group_array( cast(Type as text) || ' ' || cast(Location as TEXT) || ' ' || cast(Description as TEXT))

       from (select Type, Location, Description from SCSexOffenders_smts smts
             where smts.id = SCSexOffenders_main.id
               and smts.state = SCSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from SCSexOffenders_photos azp
             where azp.id = SCSexOffenders_main.id
               and azp.state = SCSexOffenders_main.state)) as photos

from SCSexOffenders_main
