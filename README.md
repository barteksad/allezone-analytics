asinfo -v 'tip:host=10.0.15.4;port=3002'
docker inspect   -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' 3051020ac1f4
10.0.15.4
curl -X POST http://st118vm101.rtb-lab.pl:8088/user_profiles/AAAAAAAAAAAAAAAAA?time_range=2022-03-22T12:15:00.000_2022-03-22T12:30:00.000?limit=200