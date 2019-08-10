Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,ifnull(cast(FirstName as TEXT),'') || ', ' || ifnull(cast(MiddleName as TEXT),'') || ' '
    || ifnull(cast(LastName as TEXT),'') as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(EyeColor as TEXT)   as eyes
     ,cast(HairColor as TEXT)   as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM IDSexOffenders_aliases als
            WHERE als.id = IDSexOffenders_main.id
              AND IDSexOffenders_main.state = als.state
           )
) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as TEXT) || ' ' || cast(CityStateZip as TEXT))) as addresses

     --offenses
     ,(SELECT
           json_group_array(json_object (
                   'offense', cast(Offense as Text) || ' ' || cast(Description as TEXT) ))
       FROM (SELECT Offense, Description
             FROM IDSexOffenders_offenses azo
             WHERE azo.id = IDSexOffenders_main.id
               and IDSexOffenders_main.state = azo.state
            )
) as offenses

     --scarsTattoos
     ,json_array("Update Pending") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from IDSexOffenders_photos azp
             where azp.id = IDSexOffenders_main.id
               and azp.state = IDSexOffenders_main.state)) as photos

From IDSexOffenders_main
