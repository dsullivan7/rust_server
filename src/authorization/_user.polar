same_user(user: User, action, user_resource: User) if
  action in ["read"] and
  user.UserID = user_resource.UserID;

matching_user_id(user: User, action, user_id) if
  action in ["delete", "modify"] and
  user.UserID = user_id;

allow(actor, action, resource) if
  same_user(actor, action, resource);

allow(actor, action, resource) if
  matching_user_id(actor, action, resource);
