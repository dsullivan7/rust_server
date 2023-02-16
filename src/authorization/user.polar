allow_field(user: User, action, resource: User, field) if
  user.role = "admin" or
  (user.user_id = resource.user_id and action in ["update"] and field in ["first_name", "last_name"]);
