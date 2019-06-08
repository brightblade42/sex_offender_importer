/*
    CSV Temp tables are selected into main SexOffender table.
    Each child table that that would normally be queried as a join
    or a subquery is instead imported into a single column
    as a json object or array.

    The DB is read-only and the service returns a json document.
    It makes sense to store most data as json in columns rather than
    build it up on each search.
 */
Insert into SexOffender ( id, name, age, addresses, state, aliases, offenses,
                        personalDetails,photos )
SELECT id,
       name,
       age,
       json_array(json_object('address', address)) as addresses,
       state,
       -- aliases
        (SELECT json_group_array (alias)
            FROM
            (SELECT alias
                FROM AZSexOffenders_aliases als
                WHERE als.id = AZSexOffenders_main.id
                AND AZSexOffenders_main.state = als.state
            )
        ) as aliases,
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
                    FROM AZSexOffenders_offenses azo
                    WHERE azo.id = AZSexOffenders_main.id
                    and AZSexOffenders_main.state = azo.state
                )
        ) as offenses,
        -- personal details
        (select json_group_array( json_object( 'age', age, 'eyes', eyes,
                   'hair', hair, 'height', height, 'level', level,
                   'race', race, 'scars_tattoos', scars_tattoos, 'sex', sex,
                   'status', status, 'weight', weight ))
            from (select age, eyes, hair, height, level, race, scars_tattoos,
                     sex, status, weight
                  from AZSexOffenders_main azm
                  where azm.id = AZSexOffenders_main.id
                  and azm.state = AZSexOffenders_main.state
             )) as personalDetails,

       (select json_group_array(PhotoFile)
                from (select PhotoFile from AZSexOffenders_photos azp
                     where azp.id = AZSexOffenders_main.id
                  and azp.state = AZSexOffenders_main.state)) as photos


from AZSexOffenders_main