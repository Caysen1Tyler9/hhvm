/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package thrift.test;

import java.util.List;
import java.util.Map;
import java.util.Set;
import javax.annotation.concurrent.Immutable;
import javax.annotation.Nullable;
import com.facebook.hyperthrift.HyperThriftBase;
import com.facebook.hyperthrift.reflect.HyperThriftType;

@Immutable
@HyperThriftType
public class OneField extends HyperThriftBase {
  public static final String TYPE_NAME = "thrift.test.OneField";


  @Nullable
  public thrift.test.EmptyStruct field() {
    return getFieldValue(0);
  }



  public static class Builder extends HyperThriftBase.Builder {
    public Builder() {
      super(1);
    }

    public Builder(OneField other) {
      super(other);
    }

    @Nullable
    public thrift.test.EmptyStruct field() {
      return getFieldValue(0);
    }

    public Builder OneField( thrift.test.EmptyStruct value) {
      setFieldValue(0, value);
      return this;
    }

    public OneField build() {
      Object[] fields = markBuilt();
      OneField instance = new OneField();
      instance.init(TYPE_NAME, fields);
      return instance;
    }
  }
}