Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,ifnull(LastName,'') || ', ' || ifnull(FirstName,'') || ' ' || ifnull(MiddleName,'') as name
     --'' as age,
     ,YearOfBirth as DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases TODO: MISSING TABLE
     ,json_array("Unknown") as aliases
     --addresses
     ,json_array(
        json_object('address', cast(LastKnownAddress as Text))) as addresses

     -- offenses TODO: MISSING TABLE
     ,(SELECT
           json_group_array(json_object ( 'offense', cast(Offense as Text)
               ))
       FROM (SELECT Offense
             FROM GA_SexOffenders_offenses azo
             WHERE azo.id = GA_SexOffenders_main.id
               and GA_SexOffenders_main.state = azo.state
            )
) as offenses

     --scarsTattoos
     ,json_array("Unknown") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from GA_SexOffenders_photos azp
             where azp.id = GA_SexOffenders_main.id
               and azp.state = GA_SexOffenders_main.state)) as photos

From GA_SexOffenders_main
