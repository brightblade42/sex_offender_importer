Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name
     ,DateOfBirth
     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
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
