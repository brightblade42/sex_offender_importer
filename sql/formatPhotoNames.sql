update Photos set id = RTRIM(name, ".png"), name = RTRIM(name, ".png") where state = "TX";
