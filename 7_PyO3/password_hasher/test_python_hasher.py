import password_hasher

actual_pass = "my_super!_secure_password!!1!"
hashed = password_hasher.hash_password(actual_pass)
print("Hashed password: ", hashed)

test_pass1 = "my_super!_secure_password!!!"

is_valid = password_hasher.verify_password(test_pass1, hashed)
print("Is {test_pass1} valid? ", is_valid)

test_pass2 = "my_super!_secure_password!!1!"
is_valid = password_hasher.verify_password(test_pass2, hashed)
print("Is {test_pass2} valid? ", is_valid)