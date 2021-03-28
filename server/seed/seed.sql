INSERT INTO rooms(id, name) VALUES ('3ade4e2b-f731-4e4e-a2d2-7664b1c13947', 'The Hackerspace');
INSERT INTO rooms(id, name) VALUES ('e9da3ec9-2e99-4df4-b5ad-4af057fe178e', 'Family Group');
INSERT INTO rooms(id, name) VALUES ('579fda8d-8eed-42f0-b168-cd7c9020deed', 'Super Secret Group');

-- All passwords are 123456
INSERT INTO users(id, username, password) VALUES ('fc4258e4-d67c-4717-9592-ecb4eb4d48ad','Tom','$2b$10$0qhByVzJrACNoQgyzBQZTukrfi/uiZC1nVOIGDeLAK5dg1LhGQkvy');
INSERT INTO users(id, username, password) VALUES ('2bb46a93-0ae3-4e08-9d89-0b9bff60c124','Susanne','$2b$10$tegkBwdmNKQ1f.yYlllNqu293t5oaTjr6xkVEe30Sf9D3HjZ6lf5S');
INSERT INTO users(id, username, password) VALUES ('a8d1e22e-517a-4eb0-a239-b2c390c8b20c','Jerry','$2b$10$/R2Ig6sJZ8aY/XCmZuNRiOceEDcoFjjx3eDuYs3.jZuiif53LZfFi');
INSERT INTO users(id, username, password) VALUES ('894203f3-e5c0-4d8e-a21c-3a13af469c3a','Max','$2b$10$7mlh38Su9g2SYYZidnDbGulMPwCnU5zrAiC8S2bGOO4qmUzD/1f6K');
INSERT INTO users(id, username, password) VALUES ('203e0241-5451-4db0-b25b-138d88bb415f','Charlotte','$2b$10$LtRYsleNomgsVODoM5h1d.a1Z56eZjHpCy5.Kwkr.NElT/JndLA7K');
INSERT INTO users(id, username, password) VALUES ('1af680da-68ad-4ac4-8c20-87757cac274c','Zoe','$2b$10$9SZSXPSpyi.h/2dnGiPbjeayrh89WNd/uiTpLWC79HtMxNj1KXCgS');

-- Add Tom, Max and Zoe to The Hackerspace
INSERT INTO rooms_users(user_id, room_id) VALUES ('fc4258e4-d67c-4717-9592-ecb4eb4d48ad', '3ade4e2b-f731-4e4e-a2d2-7664b1c13947');
INSERT INTO rooms_users(user_id, room_id) VALUES ('894203f3-e5c0-4d8e-a21c-3a13af469c3a', '3ade4e2b-f731-4e4e-a2d2-7664b1c13947');
INSERT INTO rooms_users(user_id, room_id) VALUES ('1af680da-68ad-4ac4-8c20-87757cac274c', '3ade4e2b-f731-4e4e-a2d2-7664b1c13947');

-- Add Susanne, Jerry, Max, Charlotte and Zoe to Family Group
INSERT INTO rooms_users(user_id, room_id) VALUES ('2bb46a93-0ae3-4e08-9d89-0b9bff60c124', 'e9da3ec9-2e99-4df4-b5ad-4af057fe178e');
INSERT INTO rooms_users(user_id, room_id) VALUES ('a8d1e22e-517a-4eb0-a239-b2c390c8b20c', 'e9da3ec9-2e99-4df4-b5ad-4af057fe178e');
INSERT INTO rooms_users(user_id, room_id) VALUES ('894203f3-e5c0-4d8e-a21c-3a13af469c3a', 'e9da3ec9-2e99-4df4-b5ad-4af057fe178e');
INSERT INTO rooms_users(user_id, room_id) VALUES ('203e0241-5451-4db0-b25b-138d88bb415f', 'e9da3ec9-2e99-4df4-b5ad-4af057fe178e');
INSERT INTO rooms_users(user_id, room_id) VALUES ('1af680da-68ad-4ac4-8c20-87757cac274c', 'e9da3ec9-2e99-4df4-b5ad-4af057fe178e');

-- Add Tom and Jerry to Super Secret Group
INSERT INTO rooms_users(user_id, room_id) VALUES ('fc4258e4-d67c-4717-9592-ecb4eb4d48ad', '579fda8d-8eed-42f0-b168-cd7c9020deed');
INSERT INTO rooms_users(user_id, room_id) VALUES ('a8d1e22e-517a-4eb0-a239-b2c390c8b20c', '579fda8d-8eed-42f0-b168-cd7c9020deed');
