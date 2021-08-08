# This example demonstrates Gradient Boosting to produce a predictive model from an ensemble
# of weak predictive models. Gradient boosting can be used for regression and classification problems.
# Here, we will train a model to tackle a diabetes regression task. We will obtain the results from
# GradientBoostingRegressor with least squares loss and 500 regression trees of depth 4.

from sklearn import datasets
import xgboost as xgb

# First we need to load the data.
diabetes = datasets.load_diabetes()
X, y = diabetes.data, diabetes.target

xgb_model = xgb.XGBClassifier(objective="binary:logistic", random_state=42, booster='gbtree')
xgb_model.fit(X, y)
xgb_model.save_model("xgb_model.bst")
