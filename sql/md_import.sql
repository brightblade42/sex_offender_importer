Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MDSexOffenders_aliases als
             WHERE als.id = MDSexOffenders_main.id
               AND MDSexOffenders_main.state = als.state)
) as "aliases"

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Address as TEXT) || ' ' || cast(city_state_zip as TEXT), 'type', cast(AddressType as Text)
                       ))

       FROM
           (SELECT Address,
                   city_state_zip,
                   AddressType
            FROM MDSexOffenders_addresses arad where arad.ID = MDSexOffenders_main.ID
                                                 and arad.state = MDSexOffenders_main.state)

) as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text), 'descr',cast(Description as Text) ))
       FROM

           (SELECT Charges as offense,
                   Description
            FROM MDSexOffenders_offenses aro
            where MDSexOffenders_main.ID = aro.ID
              and MDSexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MDSexOffenders_photos azp
             where azp.id = MDSexOffenders_main.id
               and azp.state = MDSexOffenders_main.state)) as photos

from MDSexOffenders_main
