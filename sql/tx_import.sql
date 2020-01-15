Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
Select TP.PER_IDN as id

     --there's no main name so take the first in the list.
     ,( SELECT cast(NAM_TXT as TEXT)
        FROM
            (SELECT NAM_TXT
             FROM TXNAME TN
             WHERE TN.PER_IDN = TP.PER_IDN
               AND TP.state = TN.state limit 1)
) as "name"
     , (select cast(DOB_DTE as TEXT)
        from (
                 Select DOB_DTE from TXBRTHDATE TB
                 where TP.PER_IDN = TB.PER_IDN
                   and TP.state = TB.state limit 1
             ))
                  as DateOfBirth


     ,cast(EYE_COD as TEXT) as eyes
     ,cast(HAI_COD as TEXT) as hair
     ,cast(HGT_QTY as TEXT) as height
     ,cast(WGT_QTY as TEXT) as weight
     ,cast(RAC_COD as TEXT) as race
     ,cast(SEX_COD as TEXT)   as sex
     ,trim(cast(state as TEXT)) as state


     ,( SELECT json_group_array (cast(NAM_TXT as TEXT))
        FROM
            (SELECT NAM_TXT
             FROM TXNAME TN
             WHERE TN.PER_IDN = TP.PER_IDN
               AND TP.state = TN.state)
) as "aliases"
     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(street_num as TEXT)
                       || ' ' || cast(street_name as TEXT)
                       || ' ' || cast(city as TEXT)
                       || ' ' || cast(zip as TEXT)
                       || ' ' || cast(state as TEXT)
                       ))
       FROM
           (SELECT TA.SNU_NBR as street_num,
                   TA.SNA_TXT as street_name,
                   TA.CTY_TXT as city,
                   TA.ZIP_TXT as zip,
                   TA.state as state
            FROM TXAddress TA
            where TA.IND_IDN = TP.IND_IDN
              and TA.state = TP.state

           )) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT))
               )
       FROM
           (
               SELECT IND_IDN, LEN_TXT as offense
               from TXOffense TXO
                        join TXOff_Code_Sor TOCS on TXO.COJ_COD = TOCS.COJ_COD

               where TXO.IND_IDN = TP.IND_IDN
                 and TXO.state = TP.state limit 2)
) as offenses
     , json_array() as scarsTatoos

     /*,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoID as PhotoFile from TXPhoto TXP
             where TXP.IND_IDN = TP.IND_IDN
               and TXP.state = TP.state)) as photos
*/
,(select json_group_array(cast(PhotoFile as TEXT))
    from (select DPS_NBR as PhotoFile from TXINDV TXI
        where TXI.IND_IDN = TP.IND_IDN and TXI.state = TP.state)) as photos
from TXPerson TP;
