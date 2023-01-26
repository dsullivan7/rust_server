allow_field(user: User, "update", resource: User, field) if
  user.role = 'admin' or
  (user.user_id = resource.user_id and field in ['first_name', 'last_name'])
