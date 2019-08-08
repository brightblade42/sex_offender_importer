--Insert into SexOffender(id,name,dateOfBirth, state,aliases, offenses, addresses, personalDetails, photos)
CREATE VIEW AR_SexOffender as 
select id,name,DateOfBirth, state,
( SELECT json_group_array (alias)
		FROM
		(SELECT alias
				FROM ARSexOffenders_aliases als
				WHERE als.id = ARSexOffenders_main.id
				AND ARSexOffenders_main.state = als.state)
) as "aliases",

(SELECT
		json_group_array (
			json_object ('id', id, 'state', state,'offense', offense)
		)
		FROM

		(SELECT
			id,
			state,
			offense
		    FROM ARSexOffenders_offenses aro where ARSexOffenders_main.ID = aro.ID
		    and ARSexOffenders_main.state = aro.state
		)
) as offenses,
  (SELECT
		json_group_array(  json_object ('address', ifnull(address1, '') || ifnull(address2, ''),  'type', type))
		FROM
		(SELECT address1,
				address2,
				type
				FROM ARSexOffenders_addresses arad where arad.ID = ARSexOffenders_main.ID
				and arad.state = ARSexOffenders_main.state

)) as addresses,
       (SELECT
            json_group_array (json_object('driversLicense',DriversLicenseStateNumber, 'eyes',Eyes, 'hair', Hair,
                'race',Race, 'riskLevel',RiskLevel,'scarsTattoos',ScarsTattoos, 'sex',Sex))
       FROM
        (SELECT
                DriversLicenseStateNumber,
                 Eyes,
            Hair,
            ID,
            Name,
            Race,
            RiskLevel,
            ScarsTattoos,
            Sex
                FROM

            ARSexOffenders_main arm where arm.ID = ARSexOffenders_main.id and arm.state = ARSexOffenders_main.state
   )) as personalDetails,

       (select json_group_array(PhotoFile)
                from (select PhotoFile from AZSexOffenders_photos azp
                     where azp.id = ARSexOffenders_main.id
                  and azp.state = ARSexOffenders_main.state)) as photos

from ARSexOffenders_main
