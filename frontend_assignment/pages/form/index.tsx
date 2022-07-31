import { FunctionComponent } from 'react';
import React from 'react';
import { FormValueType } from './form.types';
import { validate } from './form.validation';
import {
  TextField,
  Button
} from '@mui/material';

import { Formik, FormikHelpers } from 'formik';

const formValues: FormValueType = {
  name: 'bbcfive',
  age: 18,
  address: 'GZ',
};

const FormPage: FunctionComponent = () => {
  const submitForm = (values: FormValueType, { setSubmitting }: FormikHelpers<FormValueType>) => {
    console.log(values);
    setSubmitting(true);
  };

  return (
    <div className="w-full mx-auto mt-20 md:max-w-5xl">
      <div className=" flex flex-wrap flex-col-reverse justify-between mt-8  md:flex-row ">
        <div className="w-full mb-10 md:w-9/20 ">
          <Formik initialValues={formValues} validate={validate} onSubmit={submitForm}>
            {({ values, errors, handleChange, handleBlur, handleSubmit, touched }) => (
              <form onSubmit={handleSubmit} className="w-full md:flex-row ">
                <div>
                  <TextField
                    value={values.name}
                    helperText={touched.name && Boolean(errors.name) && 'Invalid name!'}
                    name="name"
                    error={touched.name && Boolean(errors.name)}
                    onBlur={handleBlur}
                    onChange={(e) => {
                      handleChange(e);
                    }}
                    variant="outlined"
                  />
                  <TextField
                    value={values.age}
                    helperText={touched.age && Boolean(errors.age) && 'Invalid age!'}
                    name="age"
                    error={touched.age && Boolean(errors.age)}
                    onBlur={handleBlur}
                    onChange={(e) => {
                      handleChange(e);
                    }}
                    variant="outlined"
                  />
                  <TextField
                    value={values.address}
                    helperText={touched.address && Boolean(errors.address) && 'Invalid address!'}
                    name="address"
                    error={touched.address && Boolean(errors.address)}
                    onBlur={handleBlur}
                    onChange={(e) => {
                      handleChange(e);
                    }}
                    variant="outlined"
                  />
                </div>
                <Button style={{ marginTop: "10px"}} variant="outlined" type='submit'>Submit</Button>
              </form>
            )}
          </Formik>
        </div>
      </div>
    </div>
  );
};
export default FormPage;
