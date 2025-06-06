# coding: utf-8

"""
    Algod REST API.

    API endpoint for algod operations.

    The version of the OpenAPI document: 0.0.1
    Contact: contact@algorand.com
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictStr, field_validator
from typing import Any, ClassVar, Dict, List, Optional, Union
from typing_extensions import Annotated
from algokit_algod_api.models.avm_value import AvmValue
from typing import Optional, Set
from typing_extensions import Self

class ApplicationStateOperation(BaseModel):
    """
    An operation against an application's global/local/box state.
    """ # noqa: E501
    operation: StrictStr = Field(description="Operation type. Value `w` is **write**, `d` is **delete**.")
    app_state_type: StrictStr = Field(description="Type of application state. Value `g` is **global state**, `l` is **local state**, `b` is **boxes**.", alias="app-state-type")
    key: Union[Annotated[bytes, Field(strict=True)], Annotated[str, Field(strict=True)]] = Field(description="The key (name) of the global/local/box state.")
    new_value: Optional[AvmValue] = Field(default=None, alias="new-value")
    account: Optional[StrictStr] = Field(default=None, description="For local state changes, the address of the account associated with the local state.")
    __properties: ClassVar[List[str]] = ["operation", "app-state-type", "key", "new-value", "account"]

    @field_validator('key')
    def key_validate_regular_expression(cls, value):
        """Validates the regular expression"""
        if not re.match(r"^(?:[A-Za-z0-9+\/]{4})*(?:[A-Za-z0-9+\/]{2}==|[A-Za-z0-9+\/]{3}=)?$", value):
            raise ValueError(r"must validate the regular expression /^(?:[A-Za-z0-9+\/]{4})*(?:[A-Za-z0-9+\/]{2}==|[A-Za-z0-9+\/]{3}=)?$/")
        return value

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of ApplicationStateOperation from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of new_value
        if self.new_value:
            _dict['new-value'] = self.new_value.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ApplicationStateOperation from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "operation": obj.get("operation"),
            "app-state-type": obj.get("app-state-type"),
            "key": obj.get("key"),
            "new-value": AvmValue.from_dict(obj["new-value"]) if obj.get("new-value") is not None else None,
            "account": obj.get("account")
        })
        return _obj


