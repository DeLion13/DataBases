CREATE OR REPLACE FUNCTION randomMembers()
            RETURNS void AS $$
        DECLARE
            step integer  := 0;
        BEGIN
            LOOP EXIT WHEN step > 2;
                INSERT INTO public."Goods" (good_name, departments_id, categories_id)
                VALUES (
                    (
        				CASE (random() * 6)::int
           					WHEN 0 THEN 'Organic Milk'
            				WHEN 1 THEN 'Plastic Bottle'
            				WHEN 2 THEN 'Lego Robot X21'
            				WHEN 3 THEN 'Pencil'
							WHEN 4 THEN 'Oil Oleyna'
							WHEN 5 THEN 'Charger 50W'
							WHEN 6 THEN 'Lamp 3W'
        				END
    				),
                    (
        				(random() * 6)::int + 1
    				),
					(
        				(random() * 3)::int + 1
    				)
                );
                step := step + 1;
            END LOOP ;
        END;
        $$ LANGUAGE PLPGSQL;
        SELECT randomMembers();