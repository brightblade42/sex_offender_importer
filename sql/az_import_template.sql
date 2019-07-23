select * from AZ_SexOffender;
/*
    CSV Temp tables are selected into main SexOffender table.
    Each child table that that would normally be queried as a join
    or a subquery is instead imported into a single column
    as a json object or array.

    The DB is read-only and the service returns a json document.
    It makes sense to store most data as json in columns rather than
    build it up on each search.
 */
--Insert into SexOffender ( id, name, age, addresses, state, aliases, offenses,
--                        personalDetails,photos )
--CREATE VIEW AZ_SexOffender AS
SELECT id,
       name,
       age,

       json_object('address', address) as addresses,
       state
       /*,
       -- aliases
        (SELECT json_group_array (alias)
            FROM
            (SELECT alias
                FROM AZ_SexOffenders_aliases als
                WHERE als.id = AZ_SexOffenders_main.id
                AND AZ_SexOffenders_main.state = als.state
            )
        ) as aliases

        */
      /*
       ,
        -- offenses
        (SELECT
            json_group_array(json_object ( 'offense', offense, 'state', state,
                        'conviction_state', conviction_state,
                        'date_convicted', date_convicted,
                        'release_date', release_date,
                        'details', details
            ))
            FROM (SELECT description as offense, state, conviction_state,
                    date_convicted, release_date, details
                    FROM AZ_SexOffenders_offenses azo
                    WHERE azo.id = AZ_SexOffenders_main.id
                    and AZ_SexOffenders_main.state = azo.state
                )
        ) as offenses

       */
       /*,
        -- personal details
        (select json_group_array( json_object( 'age', age, 'eyes', eyes,
                   'hair', hair, 'height', height, 'level', level,
                   'race', race, 'scars_tattoos', scars_tattoos, 'sex', sex,
                   'status', status, 'weight', weight ))
            from (select age, eyes, hair, height, level, race, scars_tattoos,
                     sex, status, weight
                  from AZ_SexOffenders_main azm
                  where azm.id = AZ_SexOffenders_main.id
                  and azm.state = AZ_SexOffenders_main.state
             )) as personalDetails

        */
/*
       ,
       (select json_group_array(PhotoFile)
                from (select PhotoFile from AZ_SexOffenders_photos azp
                     where azp.id = AZ_SexOffenders_main.id
                  and azp.state = AZ_SexOffenders_main.state)) as photos
*/

from AZ_SexOffenders_main