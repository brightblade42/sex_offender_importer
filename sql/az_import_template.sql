/*
    CSV Temp tables are selected into main SexOffender table.
    Each child table that that would normally be queried as a join
    or a subquery is instead imported into a single column
    as a json object or array.

    The DB is read-only and the service returns a json document.
    It makes sense to store most data as json in columns rather than
    build it up on each search.
 */
Insert into SexOffender(id,name,age, addresses, state,aliases, offenses, personalDetails)
select id,name,
       age,
       json_array(json_object('address', address)) as addresses,
       state,
( SELECT json_group_array (alias)
		FROM
		(SELECT alias
				FROM AZSexOffenders_aliases als
				WHERE als.id = AZSexOffenders_main.id
				AND AZSexOffenders_main.state = als.state)
) as aliases,
(SELECT
	json_group_array (
			json_object ('offense', offense, 'state', state, 'conviction_state', conviction_state, 'date_convicted', date_convicted, 'release_date', release_date, 'details', details)
		)
		FROM

		(SELECT
			description offense,
			state,
			conviction_state,
			date_convicted,
			release_date,
			details
			FROM AZSexOffenders_offenses azo
			WHERE azo.id = AZSexOffenders_main.id and AZSexOffenders_main.state = azo.state
		)
) as offenses,
       (SELECT json_group_array(
                       json_object('age', age, 'eyes', eyes, 'hair', hair, 'height', height, 'level', level, 'race',
                                   race, 'scars_tattoos',
                                   scars_tattoos, 'sex', sex, 'status', status, 'weight', weight)
                   )
        FROM (SELECT age,
                     eyes,
                     hair,
                     height,
                     level,
                     race,
                     scars_tattoos,
                     sex,
                     status,
                     weight
              FROM AZSexOffenders_main azm
              WHERE azm.id = AZSexOffenders_main.id
                and azm.state = AZSexOffenders_main.state
             )
       ) as personalDetails


       /*
(SELECT
    json_group_array (
            json_object('DateOfBirth', DateOfBirth, 'DriversLicenseStateNumber',DriversLicenseStateNumber,'Eyes',Eyes, 'Hair',Hair,
                'Race',Race,'RiskLevel',RiskLevel,'ScarsTattoos',ScarsTattoos,'Sex',Sex))
            FROM
            (SELECT
                DateOfBirth,
                DriversLicenseStateNumber,
                Eyes,
                Hair,
                Race,
                RiskLevel,
                ScarsTattoos,
                Sex

            )
			FROM AZSexOffenders_main azm
			WHERE azm.id = AZSexOffenders_main.id and AZSexOffenders_main.state = azm.state
        )
) as personalDetails
*/

from AZSexOffenders_main